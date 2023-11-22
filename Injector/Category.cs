using Newtonsoft.Json;

namespace Injector;

public class Category
{
    public long? Id { get; set; }

    [JsonProperty("name")]
    public string Name { get; set; }

    [JsonProperty("subcategories")]
    public List<Category>? Subcategories { get; set; }

    public override string ToString()
    {
        return $"Category{{Name={Name}, Subcategories=[{string.Join(", ", Subcategories)}]}}";
    }
}