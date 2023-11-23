using System.Text;
using System.Text.RegularExpressions;
using Bukimedia.PrestaSharp.Entities.AuxEntities;

namespace Injector;

public static class StringExtensions
{
    public static string Slugify(this string phrase)
    {
        var str = phrase.ToLower().AsciifyDiacritics();
        str = Regex.Replace(str, @"[^a-z0-9\s-]", "");
        str = Regex.Replace(str, @"\s+", " ").Trim();
        str = Regex.Replace(str, @"\s", "-");
        return str;
    }

    public static string ToPrestaName(this string phrase)
    {
        return phrase.Replace("#", "sharp")
                .Replace("<", "")
                .Replace(">", "")
                .Replace(";", "")
                .Replace("=", "")
                .Replace("{", "")
                .Replace("}", "");
    }

    public static string FilterDescription(this string description)
    {
        return description.Replace(";", "")
                .Replace(":", "")
                .Replace("=", "")
                .Replace("{", "")
                .Replace("}", "");
    }

    public static string FilterToParsableDecimal(this string phrase)
    {
        var text = Regex.Replace(phrase, @"[^0-9,.]", "");
        if (text.Length == 0)
        {
            return "0";
        }

        return text;
    }

    public static string AsciifyDiacritics(this string phrase)
    {
        var builder = new StringBuilder();
        var characters = phrase.ToCharArray().ToList();
        characters.ForEach(c =>
        {
            if (c == 'ą')
            {
                builder.Append('a');
            }

            else if (c == 'ć')
            {
                builder.Append('c');
            }

            else if (c == 'ę')
            {
                builder.Append('e');
            }

            else if (c == 'ł')
            {
                builder.Append('l');
            }

            else if (c == 'ń')
            {
                builder.Append('n');
            }

            else if (c == 'ó')
            {
                builder.Append('o');
            }

            else if (c == 'ś')
            {
                builder.Append('s');
            }

            else if (c == 'ż')
            {
                builder.Append('z');
            }

            else if (c == 'ź')
            {
                builder.Append('z');
            }
            else
            {
                builder.Append(c);
            }
        });
        return builder.ToString();
    }

    public static List<language> ToLanguageList(this string value)
    {
        return new List<language>
        {
                new(1, value),
        };
    }
}