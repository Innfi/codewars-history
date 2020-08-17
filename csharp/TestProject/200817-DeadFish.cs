/*
 https://www.codewars.com/kata/51e0007c1f9378fa810002a9
 */

using System.Collections.Generic;
using Microsoft.VisualStudio.TestTools.UnitTesting;


namespace DeadFish
{
    public class Kata
    {
        public static int[] Parse(string data)
        {
            var numbers = new List<int>();
            var i = 0;

            foreach (var letter in data)
            {
                switch (letter)
                {
                    case 'i': i += 1; break;
                    case 'd': i -= 1; break;
                    case 's': i *= i; break;
                    case 'o': numbers.Add(i); break;
                    default: break;
                }
            }

            return numbers.ToArray();
        }
    }

    [TestClass]
    public class TestDeadFish
    {
        [TestMethod]
        public void Test0PartialCase()
        {
            AssertEqual(Kata.Parse("iio"), new int[] { 2 });
        }

        [TestMethod]
        public void Test1SimpleCase()
        {
            AssertEqual(Kata.Parse("iiisdoso"), new int[] { 8, 64 });
        }

        protected void AssertEqual(int[] lhs, int[] rhs)
        {
            Assert.AreEqual(lhs.Length, rhs.Length);
            for (int i = 0; i < lhs.Length; i++)
            {
                Assert.AreEqual(lhs[i], rhs[i]);
            }
        }
    }
}
