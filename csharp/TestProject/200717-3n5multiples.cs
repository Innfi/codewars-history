/*
 https://www.codewars.com/kata/514b92a657cdc65150000006

one of the optimal solutions:

    using System.Linq;
public static class Kata
{
  public static int Solution(int n)
  {
    return Enumerable.Range(0, n).Where(e => e % 3 == 0 || e % 5 == 0).Sum();
  }
}

*/

using Microsoft.VisualStudio.TestTools.UnitTesting;


namespace KataTest
{
    public class Kata
    {
        public static int Solution(int value)
        {
            return ToMultiplierSum(3, value) + ToMultiplierSum(5, value)
                - ToMultiplierSum(15, value);
        }

        public static int ToMultiplierSum(int multiplier, int value)
        {
            var sum = 0;
            var counter = 1;

            while (multiplier * counter < value) sum += multiplier * counter++;

            return sum;
        }
    }

    [TestClass]
    public class Tests
    {
        [TestMethod]
        public void Test2BasicTest()
        {
            Assert.AreEqual(Kata.Solution(10), 23);
        }
    }
}
