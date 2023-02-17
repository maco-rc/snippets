using System;

namespace Solution.Kata
{
    public class SolutionKata
    {
        public static int binaryArrayToNumber(int[] BinaryArray)
        {
            //Code here
            var base10 = Convert
                        .ToInt32(String
                        .Join("", BinaryArray), 2);

            return base10;
        }
    }
}