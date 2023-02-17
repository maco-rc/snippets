using Xunit;
using Solution.Kata;

namespace Solution.Tests;

public class SolutionTests
{
    string expected = "camel Casing";
    [Fact]
    public void Sample() => Assert.Equal(expected, SolutionKata.BreakCamelCase("camelCasing"));
}