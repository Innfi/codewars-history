/*
https://www.codewars.com/kata/5202ef17a402dd033c000009

*/

namespace KataTest
{
    using Microsoft.VisualStudio.TestTools.UnitTesting;
    using System.Collections.Generic;
    using System.Linq;

    public class Kata
    {
        public static string TitleCase(string title, string minorWords = "")
        {
            var minorWordsSet = new HashSet<string>();
            if (!string.IsNullOrEmpty(minorWords)) minorWordsSet =
                     new HashSet<string>(minorWords.ToLower().Split(' '));

            return string.Join(" ", title.ToLower().Split(' ')
                .Select((x, index) => ToTitleWord(x, index, minorWordsSet)).ToArray());
        }

        public static string ToTitleWord(string word, int index, HashSet<string> minorWords)
        {
            if (index > 0 && minorWords.Contains(word)) return word;

            return ToTitleWord(word);
        }

        public static string ToTitleWord(string word)
        {
            if (string.IsNullOrEmpty(word)) return word;

            return word[0].ToString().ToUpper() + word.Substring(1);
        }
    }

    public class KataOld
    {
        public static string TitleCase(string title, string minorWords = "")
        {
            if (string.IsNullOrEmpty(title)) return title;

            var words = title.ToLower().Split(' ');

            var minorWordsSet = new HashSet<string>();
            if(!string.IsNullOrEmpty(minorWords)) minorWordsSet = 
                    new HashSet<string>(minorWords.ToLower().Split(' '));

            for (var i = 0; i < words.Length; i++)
            {
                if (i > 0 && minorWordsSet.Contains(words[i])) continue;

                words[i] = ToTitleWord(words[i]);
            }

            return string.Join(" ", words);
        }

        public static string ToTitleWord(string word)
        {
            if (string.IsNullOrEmpty(word)) return word;

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
        public void Test0JoinString()
        {
            Assert.AreEqual(string.Join(" ", testString1.Split(' ')), testString1);
        }

        [TestMethod]
        public void Test0HashSet()
        {
            var minorWordsSet = new HashSet<string>(minorWords1.Split(' '));

            Assert.AreEqual(minorWordsSet.Contains("a"), true);
            Assert.AreEqual(minorWordsSet.Contains("A"), false);
        }

        [TestMethod]
        public void Test0SubstringBoundary()
        {
            string testBoundary = "aa";

            Assert.AreEqual(testBoundary.Substring(testBoundary.Length), "");
        }

        [TestMethod]
        [ExpectedException(typeof(System.ArgumentOutOfRangeException))]
        public void Test0SubstringIndex()
        {
            string testIndex = "anyRandomText";

            Assert.AreEqual(testIndex.Substring(testIndex.Length+1), "");
        }

        [TestMethod]
        public void Test0Linq()
        {
            var capitalLetters = "HELLO TO THE WORLD";
            var words = capitalLetters.ToLower().Split(' ');
            var firstArray = words.Select(x => Kata.ToTitleWord(x)).ToArray();

            Assert.AreEqual(string.Join(" ", firstArray), "Hello To The World");
        }

        [TestMethod]
        public void Test0LinqWithEdgeCases()
        {
            var minorWordsSet = new HashSet<string>(minorWords1.ToLower().Split(' '));
            var resultArray = testString1.ToLower().Split(' ')
                .Select((x, index) => Kata.ToTitleWord(x, index, minorWordsSet)).ToArray();

            Assert.AreEqual(string.Join(" ", resultArray), "A Clash of Kings");
        }

        [TestMethod]
        public void Test1ToTitleWord()
        {
            Assert.AreEqual(Kata.ToTitleWord("red"), "Red");
        }

        [TestMethod]
        public void Test1ToTitleWordSingleLetter()
        {
            Assert.AreEqual(Kata.ToTitleWord("r"), "R");
        }

        [TestMethod]
        public void Test1ToTitleWordEmptyString()
        {
            Assert.AreEqual(Kata.ToTitleWord(""), "");
        }

        [TestMethod]
        public void Test2FirstRule()
        {
            Assert.AreEqual(Kata.TitleCase(testString1), "A Clash Of Kings");
        }

        [TestMethod]
        public void Test2FirstRuleWithMinorWords()
        {
            Assert.AreEqual(Kata.TitleCase(testString1, minorWords1), "A Clash of Kings");
        }

        [TestMethod]
        public void Test2SecondRuleIgnorMinorWordsCase()
        {
            Assert.AreEqual(Kata.TitleCase("THE WIND IN THE WILLOWS", "The In"), "The Wind in the Willows");
        }

        [TestMethod]
        public void Test2EmptyTitle()
        {
            Assert.AreEqual(Kata.TitleCase("", "a b c"), "");
        }
    }
}