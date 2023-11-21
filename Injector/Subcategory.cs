using Newtonsoft.Json;

namespace Injector;

public class Subcategory
{
    [JsonProperty("name")]
    public string Name { get; set; }

    [JsonProperty("url")]
    public string Url { get; set; }

    public override string ToString()
    {
        return $"Subcategory{{Name={Name}, Url={Url}}}";
    }
}