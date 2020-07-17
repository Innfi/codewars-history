using Microsoft.VisualStudio.TestTools.UnitTesting;

/*
 https://www.codewars.com/kata/514b92a657cdc65150000006
*/


namespace KataTest
{
    public class Kata
    {
        public static int Solution(int value)
        {
            var sum = 0;

            sum += ToMultiplierSum(3, value);
            sum += ToMultiplierSum(5, value);

            return sum;
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
