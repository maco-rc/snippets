using System;

namespace Solution.Kata;
public class SolutionKata
{
    public static string BreakCamelCase(string str)
    {
        string[] camelcase = System.Text.RegularExpressions.Regex.Split(str, "(?=[A-Z])");
        return String.Join(" ", camelcase);
    }
}
