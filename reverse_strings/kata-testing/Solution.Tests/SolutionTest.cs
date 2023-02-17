using Xunit;
using Solution.Kata;

namespace Solution.UnitTests.Tests;

public class Kata
{
    [Fact]
    public void World()
    {
      Assert.Equal("dlrow", SolutionKata.Solution("world"));
    }
}