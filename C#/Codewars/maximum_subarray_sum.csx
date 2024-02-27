public static class Kata
{
    public static int MaxSequence(int[] arr) {
        int max = 0;
        int sum = 0;
        for (int i = 0; i < arr.Length; i++) {
            sum += arr[i];
            if (sum < 0) sum = 0;
            if (sum > max) max = sum;
        }
        return max;
    }
}

WriteLine(6 == Kata.MaxSequence(new int[] { -2, 1, -3, 4, -1, 2, 1, -5, 4 }));
WriteLine(0 == Kata.MaxSequence(new int[0]));
