public class MyStack
{
    Queue<int> q;

    public MyStack() {
        q = [];
    }

    public void Push(int x) {
        q.Enqueue(x);
        for (int i = 0; i < q.Count - 1; i++) {
            q.Enqueue(q.Dequeue());
        }
    }

    public int Pop() {
        return q.Dequeue();
    }

    public int Top() {
        return q.Peek();
    }

    public bool Empty() {
        return q.Count == 0;
    }
}