using Newtonsoft.Json;
using NLog;

namespace Injector;

public class Product
{
    private static ILogger Logger { get; } = LogManager.GetCurrentClassLogger();

    [JsonProperty("id")]
    public int Id { get; set; }

    [JsonProperty("category")]
    public string Category { get; set; }

    [JsonProperty("subcategory")]
    public string Subcategory { get; set; }

    [JsonProperty("Kolor")]
    [OptionalProductProperty]
    public string? Color { get; set; }

    public Dictionary<string, string> GetOptionalProperties()
    {
        var propertyInfos = typeof(Product).GetProperties();
        var result = new Dictionary<string, string>();

        foreach (var propertyInfo in propertyInfos)
        {
            string jsonPropertyName = null;
            var attributes = propertyInfo.GetCustomAttributes(true);
            var isOptional = false;
            foreach (var attribute in attributes)
            {
                switch (attribute)
                {
                    case OptionalProductPropertyAttribute:
                        isOptional = true;
                        break;
                    case JsonPropertyAttribute property:
                        jsonPropertyName = property.PropertyName;
                        break;
                }
            }

            if (!isOptional)
            {
                continue;
            }

            if (jsonPropertyName == null)
            {
                throw new Exception("JSON property should have a name");
            }

            var propertyValue = propertyInfo.GetValue(this) as string;
            if (propertyValue == null)
            {
                continue;
            }

            Logger.Trace("Adding property value: {}: {}", jsonPropertyName, propertyValue);
            result[jsonPropertyName] = propertyValue;
        }

        return result;
    }

    public override string ToString()
    {
        return $"Product{{id={Id}}}";
    }
}