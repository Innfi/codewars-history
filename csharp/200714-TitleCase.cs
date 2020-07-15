/*
https://www.codewars.com/kata/5202ef17a402dd033c000009

*/

namespace KataTest
{
    using Microsoft.VisualStudio.TestTools.UnitTesting;
    using System.Collections.Generic;

    public class Kata
    {
        public static string TitleCase(string title, string minorWords = "")
        {
            var words = title.ToLower().Split(' ');

            var minorWordsSet = new HashSet<string>();
            if(!string.IsNullOrEmpty(minorWords)) minorWordsSet = new HashSet<string>(minorWords.Split(' '));

            for (var i = 0; i < words.Length; i++)
            {
                if (i > 0 && minorWordsSet.Contains(words[i])) continue;

                words[i] = ToTitleWord(words[i]);
            }

            return string.Join(" ", words);
        }

        protected static string ToTitleWord(string word)
        {
            return word[0].ToString().ToUpper() + word.Substring(1);
        }
    }

    [TestClass]
    public class TestString
    {
        public string testString1 = "a clash of KINGS";
        public string minorWords1 = "a an the of";

        [TestMethod]
        public void Test0ToLowerCase()
        {
            Assert.AreEqual(testString1.ToLower(), "a clash of kings");
        }

        [TestMethod]
        public void Test1JoinString()
        {
            Assert.AreEqual(string.Join(" ", testString1.Split(' ')), testString1);
        }

        [TestMethod]
        public void Test2HashSet()
        {
            var minorWordsSet = new HashSet<string>(minorWords1.Split(' '));

            Assert.AreEqual(minorWordsSet.Contains("a"), true);
            Assert.AreEqual(minorWordsSet.Contains("A"), false);
        }

        [TestMethod]
        public void Test3SubstringBoundary()
        {
            string testBoundary = "aa";

            Assert.AreEqual(testBoundary.Substring(testBoundary.Length), "");
        }

        [TestMethod]
        public void Test100FirstRule()
        {
            Assert.AreEqual(Kata.TitleCase(testString1), "A Clash Of Kings");
        }

        [TestMethod]
        public void Test101FirstRuleWithMinorWords()
        {
            Assert.AreEqual(Kata.TitleCase(testString1, minorWords1), "A Clash of Kings");
        }
    }
}
