using System;

namespace Solution;
public static class Kata
{
    readonly static Dictionary<char, int> alphabet = new Dictionary<char, int> {
        { 'a' , 1 },  { 'b' , 2  }, { 'c' , 3 },
        { 'd' , 4 },  { 'e', 5   }, { 'f' , 6 },
        { 'g' , 7 },  { 'h' , 8  }, { 'i' , 9 },
        { 'j' , 10 }, { 'k' , 11 }, { 'l' , 12 },
        { 'm' , 13 }, { 'n', 14  }, { 'o' , 15 },
        { 'p' , 16 }, { 'q' , 17 }, { 'r' , 18 },
        { 's' , 19 }, { 't' , 20 }, { 'u' , 21 },
        { 'v' , 22 }, { 'w' , 23 }, { 'x' , 24 },
        { 'y', 25 },  { 'z' , 26 }
    };
    
    public static string AlphabetPosition(string text)
    {
        char[] list = text.ToCharArray();
        List<int> idList = new List<int>();

        foreach (var item in list)
        {
            int Id;

            var x = alphabet.TryGetValue(Char.ToLower(item), out Id);

            if (x)
            {
                idList.Add(Id);
            }
        }

        text = String.Join(" ", idList);

        return text;
    }
}
