/*
 https://www.codewars.com/kata/545cedaa9943f7fe7b000048
 */

using System.Linq;
using System.Collections.Generic;
using Microsoft.VisualStudio.TestTools.UnitTesting;


namespace Pangram
{
    public class Kata
    {
        public static bool IsPangram(string str)
        {
            var letterDict = new bool['z'-'a'+1];

            foreach (var letter in str.ToLower())
            {
                if (char.IsLetter(letter)) letterDict[(int)letter-'a'] = true;
            }

            foreach (var item in letterDict)
            {
                if (item == false) return false;
            }

            return true;
        }
    }

    [TestClass]
    public class TestPangram
    {        
        public string text = "The quick brown fox jumps over the lazy dog.";

        [TestMethod]
        public void Test1SimpleCase()
        {
            Assert.AreEqual(Kata.IsPangram(text), true);
        }
    }
}
