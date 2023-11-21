using Newtonsoft.Json;

namespace Injector;

public class Product
{
    [JsonProperty("id")]
    public int Id { get; set; }

    [JsonProperty("category")]
    public string Category { get; set; }

    [JsonProperty("subcategory")]
    public string Subcategory { get; set; }

    [JsonProperty("Kolor")]
    public string? Color { get; set; }

    public override string ToString()
    {
        return $"Product{{id={Id}}}";
    }
}