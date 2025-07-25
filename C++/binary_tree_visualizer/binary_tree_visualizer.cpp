#include <iostream>
#include <queue>

struct TreeNode {
    int value;
    TreeNode* left;
    TreeNode* right;
    TreeNode(int v) : value(v), left(nullptr), right(nullptr) {}
};

class BinaryTree {
public:
    TreeNode* root;
    BinaryTree() : root(nullptr) {}

    void insert(int value) {
        if (!root) {
            root = new TreeNode(value);
        } else {
            insertHelper(root, value);
        }
    }

    void printLevelOrder() const {
        if (!root) return;

        std::queue<TreeNode*> q;
        q.push(root);
        while (!q.empty()) {
            int levelSize = q.size();
            for (int i = 0; i < levelSize; ++i) {
                TreeNode* node = q.front(); q.pop();
                std::cout << node->value << " ";
                if (node->left) q.push(node->left);
                if (node->right) q.push(node->right);
            }
            std::cout << std::endl;
        }
    }

private:
    void insertHelper(TreeNode* node, int value) {
        if (value < node->value) {
            if (node->left) {
                insertHelper(node->left, value);
            } else {
                node->left = new TreeNode(value);
            }
        } else {
            if (node->right) {
                insertHelper(node->right, value);
            } else {
                node->right = new TreeNode(value);
            }
        }
    }
};

int main() {
    BinaryTree tree;
    int values[] = {15, 10, 20, 8, 12, 17, 25};
    for (int v : values) {
        tree.insert(v);
    }

    std::cout << "Level Order Traversal of the Binary Tree:" << std::endl;
    tree.printLevelOrder();

    return 0;
}
