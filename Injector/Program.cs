using System.Net;
using Bukimedia.PrestaSharp.Entities.AuxEntities;
using Bukimedia.PrestaSharp.Factories;
using Config.Net;
using Newtonsoft.Json;
using NLog;
using category = Bukimedia.PrestaSharp.Entities.category;
using product = Bukimedia.PrestaSharp.Entities.product;

namespace Injector;

public static class Program
{
    private const int MainPageCategoryParent = 2;
    private const string ConfigurationFile = "injector_settings.json";
    private const float VAT_MULTIPLIER = 1.23f;

    private static ILogger Logger { get; } = LogManager.GetCurrentClassLogger();

    public static void Main()
    {
        var settings = new ConfigurationBuilder<InjectorSettings>()
                .UseJsonFile(ConfigurationFile)
                .Build();

        var categoriesString = File.ReadAllText(settings.CategoriesFilePath);
        var productsString = File.ReadAllText(settings.ScrappedFilePath);

        var categories = JsonConvert.DeserializeObject<List<Category>>(categoriesString) ??
                throw new Exception("Could not deserialize categories");
        var products = JsonConvert.DeserializeObject<List<Product>>(productsString) ??
                throw new Exception("Could not deserialize products");

        // XDDDD
        ServicePointManager.ServerCertificateValidationCallback = delegate { return true; };

        var categoryFactory = new CategoryFactory(settings.PrestaShopBaseUrl, settings.PrestaShopAccount,
                settings.PrestaShopSecretKey);
        var productFactory = new ProductFactory(settings.PrestaShopBaseUrl, settings.PrestaShopAccount,
                settings.PrestaShopSecretKey);
        var productOptionFactory = new ProductOptionFactory(settings.PrestaShopBaseUrl, settings.PrestaShopAccount,
                settings.PrestaShopSecretKey);
        var productOptionValueFactory = new ProductOptionValueFactory(settings.PrestaShopBaseUrl,
                settings.PrestaShopAccount,
                settings.PrestaShopSecretKey);
        var productFeatureFactory = new ProductFeatureFactory(settings.PrestaShopBaseUrl, settings.PrestaShopAccount,
                settings.PrestaShopSecretKey);
        var productFeatureValueFactory = new ProductFeatureValueFactory(settings.PrestaShopBaseUrl,
                settings.PrestaShopAccount,
                settings.PrestaShopSecretKey);
        var combinationFactory = new CombinationFactory(settings.PrestaShopBaseUrl,
                settings.PrestaShopAccount,
                settings.PrestaShopSecretKey);
        var stockAvailableFactory = new StockAvailableFactory(settings.PrestaShopBaseUrl,
                settings.PrestaShopAccount,
                settings.PrestaShopSecretKey);

        var productOptionMapping = new ProductOptionMapping();

        ClearProducts(productFactory);
        ClearProductOptions(productOptionFactory);
        ClearProductFeatures(productFeatureFactory);
        ClearCategories(categoryFactory);
        InsertCategoryTree(categoryFactory, categories, MainPageCategoryParent);
        productOptionMapping.Insert(productFeatureFactory, productFeatureValueFactory, products);
        AddProducts(productFactory, productOptionMapping, products, categories);
        UpdateStockCount(productFactory, stockAvailableFactory);
    }

    private static void ClearProducts(ProductFactory productFactory)
    {
        var products = productFactory.GetAll();
        foreach (var product in products)
        {
            Logger.Info("Removing product {}", product.name[0].Value);
            try
            {
                productFactory.Delete(product);
            }
            catch (Exception)
            {
                Logger.Error("Could not remove product {}", product.name[0].Value);
            }
        }
    }

    private static void ClearProductOptions(ProductOptionFactory productOptionFactory)
    {
        var products = productOptionFactory.GetAll();
        foreach (var optionProduct in products)
        {
            Logger.Info("Removing product option {}", optionProduct.name[0].Value);
            try
            {
                productOptionFactory.Delete(optionProduct);
            }
            catch (Exception)
            {
                Logger.Error("Could not remove product option {}", optionProduct.name[0].Value);
            }
        }
    }

    private static void ClearCategories(CategoryFactory categoryFactory)
    {
        var categories = categoryFactory.GetAll();
        foreach (var category in categories)
        {
            if (category.id <= MainPageCategoryParent)
            {
                continue;
            }

            Logger.Info("Removing category {}", category.name[0].Value);
            try
            {
                categoryFactory.Delete(category);
            }
            catch (Exception)
            {
                Logger.Warn("Could not remove category {}. It is probably already removed.", category.name);
            }
        }
    }

    private static void ClearProductFeatures(ProductFeatureFactory productFeatureFactory)
    {
        var features = productFeatureFactory.GetAll();
        foreach (var feature in features)
        {
            Logger.Info("Removing product feature {}", feature.name[0].Value);
            try
            {
                productFeatureFactory.Delete(feature);
            }
            catch (Exception)
            {
                Logger.Error("Could not remove product feature {}", feature.name[0].Value);
            }
        }
    }

    private static void InsertCategoryTree(CategoryFactory categoryFactory, List<Category> categories, long parentId)
    {
        foreach (var category in categories)
        {
            var cat = new category
            {
                    active = 1,
                    id_parent = parentId,
                    name = category.Name.ToLanguageList(),
                    link_rewrite = category.Name.Slugify().ToLanguageList(),
            };

            var insertedCategory = categoryFactory.Add(cat);
            var id = insertedCategory.id;

            if (id == null)
            {
                Logger.Error("Failed inserting category {}", category.Name);
                continue;
            }

            Logger.Info("Inserted a category {} with parent ID = {}. Resulted in ID = {}", category.Name,
                    parentId, id);

            category.Id = id;

            InsertCategoryTree(categoryFactory, category.Subcategories, id.Value);
        }
    }

    private static void AddProducts(ProductFactory productFactory,
            ProductOptionMapping optionMapping,
            List<Product> products,
            List<Category> categories)
    {
        foreach (var product in products)
        {
            double price;
            try
            {
                price = product.ParsePrice();
            }
            catch (Exception)
            {
                Logger.Error("Could not parse price value. Price = {}", product.Price);
                continue;
            }

            var categoryId = categories
                    .Where(category => product.Category == category.Name)
                    .SelectMany(category => category.Subcategories)
                    .Where(subcategory => product.Subcategory == subcategory.Name)
                    .Select(subcategory => subcategory.Id)
                    .First();

            Logger.Error("Category ID: {}", categoryId);

            if (categoryId == null)
            {
                Logger.Warn("Could not find category for product {}", product.Id);
                continue;
            }

            var prod = new product
            {
                    active = 1,
                    state = 1,
                    name = "TODO".ToLanguageList(),
                    link_rewrite = $"TODO_{product.Id}".ToLanguageList(),
                    available_for_order = 1,
                    price = decimal.Round(new decimal(price / VAT_MULTIPLIER), 2),
                    id_tax_rules_group = 1,
                    visibility = "both",
                    type = "simple",
                    show_price = 1,
                    minimal_quantity = 1,
                    id_category_default = categoryId,
                    description = "TODO".ToLanguageList(),
                    description_short = "TODO".ToLanguageList(),
                    associations = new AssociationsProduct
                    {
                            categories = new List<Bukimedia.PrestaSharp.Entities.AuxEntities.category>
                            {
                                    new((long)categoryId),
                            },
                            product_features = optionMapping.GetFeatureListForProduct(product),
                    },
            };

            long insertedProductId;
            try
            {
                var insertedProduct = productFactory.Add(prod);
                insertedProductId = (long)insertedProduct.id!;
            }
            catch (Exception ex)
            {
                Logger.Error("Could not insert a product. ID = {}", ex, product.Id);
                continue;
            }

            Logger.Info("Inserted a product {}. Resulted in ID = {}", product.Id, insertedProductId);
            product.insertedId = insertedProductId;
        }
    }

    private static void UpdateStockCount(ProductFactory productFactory, StockAvailableFactory stockAvailableFactory)
    {
        List<product> products = productFactory.GetAll();
        var random = new Random();
        foreach (var product in products)
        {
            var quantity = random.Next(0, 10);
            foreach (var associationsStockAvailable in product.associations.stock_availables)
            {
                try
                {
                    var stockAvailable = stockAvailableFactory.Get(associationsStockAvailable.id);
                    stockAvailable.quantity = quantity;
                    stockAvailableFactory.Update(stockAvailable);
                }
                catch (Exception ex)
                {
                    Logger.Error("Could not update quantity of product {}", ex, product.id);
                    continue;
                }

                Logger.Info("Updated product {} quantity to {}", product.id, quantity);
            }
        }
    }
}