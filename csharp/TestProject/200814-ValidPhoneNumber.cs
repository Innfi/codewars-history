/*
 https://www.codewars.com/kata/525f47c79f2f25a4db000025
 */


using System.Text.RegularExpressions;
using Microsoft.VisualStudio.TestTools.UnitTesting;

//
namespace ValidPhoneNumber
{
    public class Kata
    {
        public static bool ValidPhoneNumber(string phoneNumber)
        {
            var matches = new Regex(@"^\(\d{3}\)\s{1}\d{3}-{1}\d{4}$", RegexOptions.Compiled)
                .Match(phoneNumber);
            return matches.Success && (matches.Groups.Count == 1);
        }
    }

    [TestClass]
    public class TestValidPhoneNumber
    {
        [TestMethod]
        public void Test0RegularExpressionArbitraryText()
        {
            string matchString1 = "(abc)";
            string matchString2 = "(defg)";
            string text = $"123 {matchString1} 4567 {matchString2} 22";
            var rx = new Regex(@"\([a-z]+\)", RegexOptions.Compiled | RegexOptions.IgnoreCase);

            var matches = rx.Matches(text);

            Assert.AreEqual(matches.Count, 2);
            Assert.AreEqual(matches[0].Value, matchString1);
            Assert.AreEqual(matches[1].Value, matchString2);
        }

        [TestMethod]
        public void Test1RegularExpressionNumbers()
        {
            string text = "(123) 456-7890";
            var rx = new Regex(@"\(\d{3}\) \d{3}-\d{4}", RegexOptions.Compiled);

            var matches = rx.Matches(text);
            Assert.AreEqual(matches[0].Value, text);
        }

        [TestMethod]
        public void Test1SimpleCases()
        {
            //Assert.AreEqual(Kata.ValidPhoneNumber("(123) 456-7890"), true);
            Assert.AreEqual(Kata.ValidPhoneNumber("(123) 456-7890     "), false);
            //Assert.AreEqual(Kata.ValidPhoneNumber("(123)     456-7890"), false);
            //Assert.AreEqual(Kata.ValidPhoneNumber("(1111)555 2345"), false);
            //Assert.AreEqual(Kata.ValidPhoneNumber("(098) 123 4567"), false);
        }
    }
}
