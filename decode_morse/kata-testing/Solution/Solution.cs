using System;
using System.Collections.Generic;

namespace Solution.Kata;
public class SolutionKata
{
    public static string DecodeMorse(string code)
    {
        string[] aux = code.Split("");
        List<string> l = new List<string>();

        foreach (var item in aux)
        {
            l.Add(MorseCode.Get(item));
        }
        string[] result = l.ToArray();
        string a = String.Concat(result); 

        return a;
    }
}
