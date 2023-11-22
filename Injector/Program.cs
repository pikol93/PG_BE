using System.Net;
using Bukimedia.PrestaSharp.Entities;
using Bukimedia.PrestaSharp.Factories;
using Config.Net;
using Newtonsoft.Json;
using NLog;

namespace Injector;

public static class Program
{
    private const int MainPageCategoryParent = 2;
    private const string ConfigurationFile = "injector_settings.json";

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
        var flattenedCategories = categories.SelectMany(category => category.Subcategories);

        // XDDDD
        ServicePointManager.ServerCertificateValidationCallback = delegate { return true; };

        var categoryFactory = new CategoryFactory(settings.PrestaShopBaseUrl, settings.PrestaShopAccount,
                settings.PrestaShopSecretKey);

        ClearCategories(categoryFactory);
        InsertCategoryTree(categoryFactory, categories, MainPageCategoryParent);
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

            Logger.Debug("Clearing category \"{}\"", category.name[0].Value);
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

            Logger.Info("Tried to insert a category {} with parent ID = {}. Resulted in ID = {}", category.Name,
                    parentId, id);

            if (id == null)
            {
                Logger.Error("Failed inserting category {}", category.Name);
                continue;
            }

            category.Id = id;

            InsertCategoryTree(categoryFactory, category.Subcategories, id.Value);
        }
    }
}