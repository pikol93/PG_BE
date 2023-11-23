using Bukimedia.PrestaSharp.Entities;
using Bukimedia.PrestaSharp.Factories;
using NLog;
using product_option_value = Bukimedia.PrestaSharp.Entities.AuxEntities.product_option_value;

namespace Injector;

public class ProductOptionMapping
{
    private static Logger Logger { get; } = LogManager.GetCurrentClassLogger();
    public Dictionary<string, ProductOption> NameToProductOptionDictionary { get; } = new();

    public void Insert(ProductFeatureFactory productFeatureFactory,
            ProductFeatureValueFactory productFeatureValueFactory,
            List<Product> products)
    {
        var tuples = Product.GetOptionalPropertyTuples();
        foreach (var (propertyName, getter) in tuples)
        {
            HashSet<string> possibleValues = new();
            foreach (var product in products)
            {
                var result = getter.Invoke(product);
                if (result == null)
                {
                    continue;
                }

                possibleValues.Add(result);
            }

            if (possibleValues.Count == 0)
            {
                Logger.Debug("Omitting product option {} because it has no possible values.", propertyName);
                continue;
            }

            var option = new product_feature()
            {
                name = propertyName.ToLanguageList(),
            };

            long featureId;
            try
            {
                featureId = (long)productFeatureFactory.Add(option).id!;
            }
            catch (Exception)
            {
                Logger.Error("Could not insert product feature. Name = {}", propertyName);
                continue;
            }

            Logger.Info("Inserted a product feature {}. Resulted in ID = {}", propertyName, featureId);

            var productFeatureValues = new List<ProductFeatureValue>();
            foreach (var possibleValue in possibleValues)
            {
                var optionValue = new product_feature_value
                {
                    id_feature = featureId,
                    value = possibleValue.ToLanguageList(),
                };

                long optionValueId;
                try
                {
                    optionValueId = (long)productFeatureValueFactory.Add(optionValue).id!;
                }
                catch (Exception)
                {
                    Logger.Error("Could not insert product feature value. Property = {}, Value = {}", propertyName,
                            possibleValue);
                    continue;
                }

                Logger.Info("Inserted a {} product feature value {}. Resulted in ID = {}", propertyName, possibleValue,
                        optionValueId);

                var productFeatureValue = new ProductFeatureValue
                {
                    Id = optionValueId,
                    Name = possibleValue,
                };

                productFeatureValues.Add(productFeatureValue);
            }

            var possibleValuesDictionary = productFeatureValues.ToDictionary(value => value.Name, value => value);
            var productOption = new ProductOption
            {
                Id = featureId,
                Name = propertyName,
                PossibleValues = possibleValuesDictionary,
            };

            NameToProductOptionDictionary.Add(propertyName, productOption);
        }
    }

    public List<Bukimedia.PrestaSharp.Entities.AuxEntities.product_feature> GetFeatureListForProduct(Product product)
    {
        var result = new List<Bukimedia.PrestaSharp.Entities.AuxEntities.product_feature>();
        foreach (var (optionName, optionValue) in product.GetOptionalProperties())
        {
            var productOption = NameToProductOptionDictionary[optionName];
            var productOptionValue = productOption.PossibleValues[optionValue];
            Logger.Trace("Got for product option = {} and value = {}. IDs = {}, {}", optionName, optionValue, productOption.Id, productOptionValue.Id);
            result.Add(new Bukimedia.PrestaSharp.Entities.AuxEntities.product_feature()
            {
                id = productOption.Id,
                id_feature_value = productOptionValue.Id,
            });
        }

        return result;
    }
}

public class ProductOption
{
    public long Id { get; init; }
    public string? Name { get; init; }
    public Dictionary<string, ProductFeatureValue>? PossibleValues { get; init; }
}

public class ProductFeatureValue
{
    public long Id { get; init; }
    public string? Name { get; init; }
}