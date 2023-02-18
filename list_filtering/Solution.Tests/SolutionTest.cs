namespace Solution.Tests;

public class UnitTest1
{
    [Fact]
    public void GetIntegersFromList_MixedValues_ShouldPass_1()
    {
        var list = new List<object>() { 1, 2, "a", "b" };
        var expected = new List<int>() { 1, 2 };
        var actual = ListFilterer.GetIntegersFromList(list);
        Assert.True(expected.SequenceEqual(actual));
    }
    [Fact]
    public void GetIntegersFromList_MixedValues_ShouldPass_2()
    {
        var list = new List<object>() { 1, "a", "b", 0, 15 };
        var expected = new List<int>() { 1, 0, 15 };
        var actual = ListFilterer.GetIntegersFromList(list);
        Assert.True(expected.SequenceEqual(actual));
    }
    [Fact]
    public void GetIntegersFromList_MixedValues_ShouldPass_3()
    {
        var list = new List<object>() { 1, 2, "aasf", "1", "123", 123 };
        var expected = new List<int>() { 1, 2, 123 };
        var actual = ListFilterer.GetIntegersFromList(list);
        Assert.True(expected.SequenceEqual(actual));
    }
}