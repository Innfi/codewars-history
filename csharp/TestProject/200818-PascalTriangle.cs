/*
 https://www.codewars.com/kata/5226eb40316b56c8d500030f

 */

using System.Collections.Generic;
using System.Linq;
using Microsoft.VisualStudio.TestTools.UnitTesting;


namespace PascalTriangle
{
    public class Kata
    {
        public static List<int> PascalsTriangle(int n)
        {
            if (n == 1) return new List<int> { 1 };

            var result = Enumerable.Repeat(0, Enumerable.Range(1, n).Sum()).ToList();
            result[0] = 1;
            var currentDepth = 2;

            for (var i = 1; i < result.Count; i++)
            {
                if (IsLeftmost(i, currentDepth)) result[i] = 1;
                else if (IsRightmost(i, currentDepth))
                {
                    result[i] = 1;
                    currentDepth++;
                }
                else
                {
                    var parentIndex = i - currentDepth;
                    result[i] = result[parentIndex] + result[parentIndex + 1];
                }
            }
            

            return result;
        }

        public static bool IsLeftmost(int index, int currentDepth)
        {
            return index == Enumerable.Range(1, currentDepth - 1).Sum();
        }

        public static bool IsRightmost(int index, int currentDepth)
        {
            return index == Enumerable.Range(1, currentDepth).Sum() - 1;
        }
    }


    [TestClass]
    public class TestPascalTriangle
    {
        [TestMethod]
        public void Test0CompareList()
        {
            var lhs = new List<int> { 1, 2, 3 };
            var rhs = new List<int> { 1, 2, 3 };

            ListEqual(lhs, rhs);
        }

        [TestMethod]
        public void Test0Join()
        {
            var test1 = new List<int> { 1, 2, 3 };
            var test2 = new List<int> { 2, 4, 5 };

            test1.AddRange(test2);

            ListEqual(test1, new List<int> { 1, 2, 3, 2, 4, 5 });
        }

        [TestMethod]
        public void Test0Range()
        {
            var seq = Enumerable.Range(1, 4).ToList();

            ListEqual(seq, new List<int> { 1, 2, 3, 4 });
        }

        [TestMethod]
        public void Test0RangeSum()
        {
            Assert.AreEqual(Enumerable.Range(1, 4).Sum(), 10);
        }

        [TestMethod]
        public void Test0Leftmost()
        {
            Assert.AreEqual(Kata.IsLeftmost(3, 3), true);
            Assert.AreEqual(Kata.IsLeftmost(6, 4), true);
            Assert.AreEqual(Kata.IsLeftmost(10, 5), true);
            Assert.AreEqual(Kata.IsLeftmost(8, 4), false);
        }

        [TestMethod]
        public void Test0Rightmost()
        {
            Assert.AreEqual(Kata.IsRightmost(5, 3), true);
            Assert.AreEqual(Kata.IsRightmost(9, 4), true);
            Assert.AreEqual(Kata.IsRightmost(14, 5), true);
            Assert.AreEqual(Kata.IsRightmost(12, 5), false);
        }

        [TestMethod]
        public void Test1InitialDepth()
        {
            ListEqual(Kata.PascalsTriangle(1), new List<int> { 1 });
        }

        [TestMethod]
        public void Test1Depth2()
        {
            ListEqual(Kata.PascalsTriangle(2), new List<int> { 1, 1, 1 });
        }

        [TestMethod]
        public void Test1CaseN()
        {
            ListEqual(Kata.PascalsTriangle(3), new List<int> { 1, 1, 1, 1, 2, 1 });
            ListEqual(Kata.PascalsTriangle(4), 
                new List<int> { 1, 1, 1, 1, 2, 1, 1, 3, 3, 1 });
            ListEqual(Kata.PascalsTriangle(5),
                new List<int> { 1, 1, 1, 1, 2, 1, 1, 3, 3, 1, 1, 4, 6, 4, 1 });
            ListEqual(Kata.PascalsTriangle(6),
                new List<int> { 1,
                    1, 1,
                   1, 2, 1,
                  1, 3, 3, 1,
                 1, 4, 6, 4, 1,
                1,5, 10,10, 5, 1 });
        }

        protected void ListEqual(List<int> lhs, List<int> rhs)
        {
            Assert.AreEqual(lhs.Count, rhs.Count);

            for (int i = 0; i < lhs.Count; i++) Assert.AreEqual(lhs[i], rhs[i]);
        }
    }
}
