using Xunit;
using Solution.Kata;
using System.IO;

namespace Solution.Tests;

public class KataTesting
{
    [Fact]
    public void sample()
    {
        try
        {
            string input = ".... . -.--   .--- ..- -.. .";
            string expected = "HEY JUDE";

            string actual = SolutionKata.DecodeMorse(input);

            Assert.Equal(expected, actual);
        }
        catch (IOException ex)
        {
            Assert.True(false, "There seems to be an error somewhere in your code. Exception message reads as follows: " + ex.Message);
        }
    }
}