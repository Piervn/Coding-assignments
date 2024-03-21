public class Solution {
    public int MajorityElement(int[] nums) {
        int count = 0;
        int result = 0;
        foreach (int num in nums) {
            if (count == 0) result = num;
            if (num == result) count++;
            else count--;
        }
        return result;
    }
}