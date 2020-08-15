/*
 https://www.codewars.com/kata/52b757663a95b11b3d00062d

 */

using System.Linq;
using Microsoft.VisualStudio.TestTools.UnitTesting;


namespace WeirdStringCase
{
    public class Kata
    {
        public static string ToWeirdCase(string s)
        {
            return s.Split(' ').Select((token) => {
                return new string(token.Select((letter, index) => {
                    if (index % 2 == 1) return char.ToLower(letter);
                    else return char.ToUpper(letter);
                }).ToArray());
            }).Aggregate((i, j) => i + " " + j);
        }
    }

    [TestClass]
    public class TestWeirdStringCase
    {
        [TestMethod]
        public void Test0Split()
        {
            var text = "this is a string";
            var tokens = text.Split(' ').ToList();

            var result = tokens.Aggregate((i, j) => i + " " + j);

            Assert.AreEqual(text, result);
        }

        [TestMethod]
        public void Test0Select()
        {
            var text = "string";
            var result = new string(text.Select((letter, index) => {
                if (index % 2 == 1) return char.ToLower(letter);
                else return char.ToUpper(letter);
            }).ToArray());

            Assert.AreEqual(result, "StRiNg");
        }

        [TestMethod]
        public void Test1SimpleCase()
        {
            Assert.AreEqual(Kata.ToWeirdCase("String"), "StRiNg");
            Assert.AreEqual(Kata.ToWeirdCase("This is a test"), "ThIs Is A TeSt");
        }
    }
}
