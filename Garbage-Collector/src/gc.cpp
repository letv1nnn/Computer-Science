#include <bits/stdc++.h>
#include "malloc.h"


struct GCObject {
    bool marked = false;
    std::vector<GCObject*> references;

    virtual ~GCObject() = default;

    virtual void mark() {
        if (marked)
            return;
        marked = true;
        for (auto *ref : references)
            if (ref) ref->mark();
    }
};

std::unordered_set<GCObject *> allocated_objects;

// A little bit changed memory allocator from the malloc that I've imlpemented;
// gc_malloc allocates memory using my own implementation of malloc
// and casts it to GCObject;
void *gc_malloc(size_t size) {
    void *raw = malloc(size);
    if (!raw) return nullptr;
    GCObject *obj = reinterpret_cast<GCObject*>(raw); // treat raw memory as GCObject;
    allocated_objects.insert(obj);
    return obj;
}

// insert an already allocated object to alloc objects;
void gc_register(GCObject* obj) {
    allocated_objects.insert(obj);
}

// Executing mark-and-sweep garbage collection
void gc_collect(const std::vector<GCObject*> &roots) {
    // Mark step
    for (auto *root : roots)
        if (root) root->mark();

    // Sweep step
    std::unordered_set<GCObject*> survivors;
    for (auto *obj : allocated_objects) {
        if (!obj->marked)
            free(obj);
        else {
            obj->marked = false;
            survivors.insert(obj);
        }
    }

    allocated_objects = std::move(survivors);
}

struct Node : GCObject {
    int value;
    Node *next;

    Node (int val)
        : value(val), next(nullptr) {}

    void mark() override {
        if (marked) return;
        marked = true;
        if (next) next->mark();
    }
};


int main() {

    Node* a = new (gc_malloc(sizeof(Node))) Node(10);
    Node* b = new (gc_malloc(sizeof(Node))) Node(20);
    a->next = b;

    std::vector<GCObject*> roots = {a};
    gc_collect(roots);

    a->next = nullptr;
    gc_collect(roots);

    gc_collect({});
    return 0;
}

