using Xunit;
using Solution.Kata;

namespace Solution.UnitTests.Kata
{


    public class Solution_Test
    {
        int[] Test1 = new int[] { 0, 0, 0, 0 };
        int[] Test2 = new int[] { 1, 1, 1, 1 };
        int[] Test3 = new int[] { 0, 1, 1, 0 };
        int[] Test4 = new int[] { 0, 1, 0, 1 };

        [Fact]
        public void BasicTesting()
        {
            Assert.Equal(0, SolutionKata.binaryArrayToNumber(Test1));
            Assert.Equal(15, SolutionKata.binaryArrayToNumber(Test2));
            Assert.Equal(6, SolutionKata.binaryArrayToNumber(Test3));
            Assert.Equal(5, SolutionKata.binaryArrayToNumber(Test4));
        }
    }
}