using Newtonsoft.Json;

namespace Injector;

public class Category
{
    [JsonProperty("name")]
    public string Name { get; set; }

    [JsonProperty("subcategories")]
    public List<Subcategory> Subcategories { get; set; }

    public override string ToString()
    {
        return $"Category{{Name={Name}, Subcategories=[{string.Join(", ", Subcategories)}]}}";
    }
}