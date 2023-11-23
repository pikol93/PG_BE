namespace Injector;

public interface InjectorSettings
{
    string CategoriesFilePath { get; }
    string ScrappedFilePath { get; }
    string ImagesDirectoryFilePath { get; }
    string PrestaShopBaseUrl { get; }
    string PrestaShopAccount { get; }
    string PrestaShopSecretKey { get; }
}