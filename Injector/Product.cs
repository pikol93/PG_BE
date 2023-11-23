using Newtonsoft.Json;
using NLog;

namespace Injector;

public class Product
{
    private static ILogger Logger { get; } = LogManager.GetCurrentClassLogger();

    public long? InsertedId { get; set; }

    [JsonProperty("id")]
    public int Id { get; set; }

    [JsonProperty("name")]
    public string Name { get; set; } = "";

    [JsonProperty("description")]
    public string Description { get; set; } = "";

    [JsonProperty("category")]
    public string Category { get; set; } = "INVALID";

    [JsonProperty("subcategory")]
    public string Subcategory { get; set; } = "INVALID";

    [JsonProperty("large_image_url")]
    public string LargeImageUrl { get; set; } = "INVALID";

    [JsonProperty("medium_image_url")]
    public string MediumImageUrl { get; set; } = "INVALID";

    [JsonProperty("price")]
    public string Price { get; set; } = "INVALID";

    [JsonProperty("Kolor")]
    [OptionalProductProperty]
    public string? Color { get; set; }

    [JsonProperty("Długość zausznika")]
    [OptionalProductProperty]
    public string? TempleLength { get; set; }

    [JsonProperty("Szerokość soczewki")]
    [OptionalProductProperty]
    public string? LensWidth { get; set; }

    [JsonProperty("Długość")]
    [OptionalProductProperty]
    public string? Length { get; set; }

    [JsonProperty("Materiał soczewki")]
    [OptionalProductProperty]
    public string? LensMaterial { get; set; }

    [JsonProperty("Waga")]
    [OptionalProductProperty]
    public string? Weight { get; set; }

    [JsonProperty("Dodatkowy kolor")]
    [OptionalProductProperty]
    public string? SecondaryColor { get; set; }

    [JsonProperty("Kolor soczewki")]
    [OptionalProductProperty]
    public string? LensColor { get; set; }

    [JsonProperty("Zakrywane oko")]
    [OptionalProductProperty]
    public string? CoveredEye { get; set; }

    [JsonProperty("Funkcja specjalna")]
    [OptionalProductProperty]
    public string? SpecialFunction { get; set; }

    [JsonProperty("Index")]
    [OptionalProductProperty]
    public string? Index { get; set; }

    [JsonProperty("Rodzaj soczewek")]
    [OptionalProductProperty]
    public string? LensType { get; set; }

    [JsonProperty("Lustrzanka")]
    [OptionalProductProperty]
    public string? UhIdk { get; set; }

    [JsonProperty("Dyscyplina")]
    [OptionalProductProperty]
    public string? Discipline { get; set; }

    [JsonProperty("Pojemność")]
    [OptionalProductProperty]
    public string? Capacity { get; set; }

    [JsonProperty("Kształt")]
    [OptionalProductProperty]
    public string? Shape { get; set; }

    [JsonProperty("Wysokość soczewki")]
    [OptionalProductProperty]
    public string? LensHeight { get; set; }

    [JsonProperty("Kolekcja")]
    [OptionalProductProperty]
    public string? Collection { get; set; }

    [JsonProperty("Szerokość mostka")]
    [OptionalProductProperty]
    public string? BridgeWidth { get; set; }

    [JsonProperty("Wymiary")]
    [OptionalProductProperty]
    public string? Dimensions { get; set; }

    [JsonProperty("Przeznaczenie")]
    [OptionalProductProperty]
    public string? Purpose { get; set; }

    [JsonProperty("Odporność na zarysowania")]
    [OptionalProductProperty]
    public string? ScratchResistance { get; set; }

    [JsonProperty("Wzór")]
    [OptionalProductProperty]
    public string? Pattern { get; set; }

    [JsonProperty("Polaryzacja")]
    [OptionalProductProperty]
    public string? Polarization { get; set; }

    [JsonProperty("Filtr UV")]
    [OptionalProductProperty]
    public string? UvFilter { get; set; }

    [JsonProperty("Szerokość oprawy")]
    [OptionalProductProperty]
    public string? BindingWidth { get; set; }

    [JsonProperty("Typ")]
    [OptionalProductProperty]
    public string? Type { get; set; }

    [JsonProperty("Termin realizacji zamówienia")]
    [OptionalProductProperty]
    public string? OrderCompletionDate { get; set; }

    [JsonProperty("Rozmiar")]
    [OptionalProductProperty]
    public string? Size { get; set; }

    [JsonProperty("Przekątna soczewki")]
    [OptionalProductProperty]
    public string? LensDiagonal { get; set; }

    [JsonProperty("Wykończenie")]
    [OptionalProductProperty]
    public string? Finishing { get; set; }

    [JsonProperty("Antyreflex")]
    [OptionalProductProperty]
    public string? Antireflex { get; set; }

    [JsonProperty("Materiał")]
    [OptionalProductProperty]
    public string? Material { get; set; }

    [JsonProperty("Kolory Minaflex")]
    [OptionalProductProperty]
    public string? MiraflexColors { get; set; }

    [JsonProperty("Odpowiednie dla wieku")]
    [OptionalProductProperty]
    public string? SuitableForAge { get; set; }

    public static List<(string, Func<Product, string?>)> GetOptionalPropertyTuples()
    {
        var propertyInfos = typeof(Product).GetProperties();
        var result = new List<(string, Func<Product, string?>)>();

        foreach (var propertyInfo in propertyInfos)
        {
            string? jsonPropertyName = null;
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

            result.Add((jsonPropertyName, Lambda));
            continue;

            string? Lambda(Product product)
            {
                return propertyInfo.GetValue(product) as string;
            }
        }

        return result;
    }

    public Dictionary<string, string> GetOptionalProperties()
    {
        var propertyInfos = typeof(Product).GetProperties();
        var result = new Dictionary<string, string>();

        foreach (var propertyInfo in propertyInfos)
        {
            string? jsonPropertyName = null;
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

    public double ParsePrice()
    {
        var curatedPriceString = Price.Replace(" zł", "").Replace(",", "");
        return double.Parse(curatedPriceString) / 100.0;
    }

    public override string ToString()
    {
        return $"Product{{id={Id}}}";
    }
}