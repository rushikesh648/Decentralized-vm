import uuid
from typing import Any, List, Optional

# --- 1. The Connection Object (Simulated) ---

class MockConnection:
    """A simulated connection object that tracks its state."""
    def __init__(self, name: str):
        self.id = uuid.uuid4()
        self.name = name
        self.is_active = False

    def connect(self):
        """Simulate establishing a connection."""
        self.is_active = True
        print(f"✅ Connection '{self.name}' ({self.id.short()}) established.")

    def close(self):
        """Simulate closing a connection."""
        if self.is_active:
            self.is_active = False
            print(f"🛑 Connection '{self.name}' ({self.id.short()}) closed.")
        else:
            print(f"🛑 Connection '{self.name}' ({self.id.short()}) already inactive.")

    def __repr__(self):
        return f"<Conn: {self.name}, Active: {self.is_active}>"

# --- 2. The Tree Node ---

class ConnectionNode:
    """A node in the tree, holding a connection and references to children."""
    def __init__(self, name: str, parent: Optional['ConnectionNode'] = None):
        self.name = name
        self.connection: Optional[MockConnection] = None
        self.children: List[ConnectionNode] = []
        self.parent = parent

    def create_connection(self):
        """Creates the mock connection for this node."""
        self.connection = MockConnection(self.name)
        self.connection.connect()

    def add_child(self, child_name: str) -> 'ConnectionNode':
        """Adds a new child node to this node."""
        new_child = ConnectionNode(child_name, parent=self)
        self.children.append(new_child)
        return new_child

    def close_all(self):
        """Recursively closes connections in a depth-first (child-first) order."""
        # 1. Close children connections first (pre-order traversal)
        for child in self.children:
            child.close_all()
        
        # 2. Close this node's connection
        if self.connection:
            self.connection.close()
        
    def __repr__(self):
        return f"Node(name='{self.name}', children={len(self.children)}, conn={self.connection is not None})"

# --- 3. Example Usage ---

# 1. Create the Root Node
root = ConnectionNode("Root_Server")
root.create_connection() # Connection for the root is established

# 2. Create a Child Node
child_db = root.add_child("Database_Pool")
child_db.create_connection()

# 3. Create a Grandchild Node
grandchild_read_replica = child_db.add_child("Read_Replica_A")
grandchild_read_replica.create_connection()

# 4. Create another Child Node at the root level
child_api = root.add_child("External_API_Client")
child_api.create_connection()

print("\n--- Current Tree State ---")
print(f"Root: {root.connection}")
print(f"  |-- Child DB: {child_db.connection}")
print(f"  |    |-- Grandchild Replica: {grandchild_read_replica.connection}")
print(f"  |-- Child API: {child_api.connection}")

# 5. Close all connections (demonstrates synchronized shutdown)
print("\n--- Closing Connections (Child-First Order) ---")
root.close_all()
