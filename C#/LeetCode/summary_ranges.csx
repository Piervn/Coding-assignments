public class Solution
{
    public IList<string> SummaryRanges(int[] nums) {
        var result = new List<string>();
        int start = 0;
        int l = nums.Length;
        for(var i = 0; i < l; ++i) {
            if (i != l - 1 && nums[i + 1] == nums[i] + 1) { continue; }
            result.Add(start == i ? nums[i].ToString() : $"{nums[start]}->{nums[i]}");
            start = i + 1;
        }
        return result;
    }
}