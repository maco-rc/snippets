namespace Solution;
public class Kata
{
  public static string AreYouPlayingBanjo(string name)
  {
    if (name.ToLower().First().Equals('r'))
    {
        return name + " plays banjo";
    } 
    else 
    {
        return name + " does not play banjo";
    }
  }
}
