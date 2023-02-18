using Solution;

namespace Solution.Tests;

public class AreYouPlayingBanjo
{
  [Fact]
  public static void Martin()
  {
    Assert.Equal("Martin does not play banjo", Kata.AreYouPlayingBanjo("Martin"));
  }
  
  [Fact]
  public static void Rikke()
  {
    Assert.Equal("Rikke plays banjo", Kata.AreYouPlayingBanjo("Rikke"));
  }
  
  [Fact]
  public static void bravo()
  {
    Assert.Equal("bravo does not play banjo", Kata.AreYouPlayingBanjo("bravo"));
  }
  
    [Fact]
    public static void rolf()
    {
        Assert.Equal("rolf plays banjo", Kata.AreYouPlayingBanjo("rolf"));
    }
}