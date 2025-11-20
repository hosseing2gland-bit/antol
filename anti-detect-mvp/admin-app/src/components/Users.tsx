import { useEffect, useState } from 'react';
import { Plus, Edit, Trash2 } from 'lucide-react';
import { useUsersStore } from '../store';

export default function Users() {
  const { users, fetchUsers, createUser, updateUser, deleteUser } = useUsersStore();
  const [showModal, setShowModal] = useState(false);
  const [editingUser, setEditingUser] = useState<any>(null);
  const [formData, setFormData] = useState({
    email: '',
    password: '',
    role: 'user',
    is_active: true,
  });

  useEffect(() => {
    fetchUsers();
  }, []);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      if (editingUser) {
        await updateUser(editingUser.id, {
          email: formData.email,
          role: formData.role,
          is_active: formData.is_active,
        });
      } else {
        await createUser(formData);
      }
      setShowModal(false);
      resetForm();
      fetchUsers();
    } catch (error) {
      console.error('Failed to save user:', error);
    }
  };

  const resetForm = () => {
    setFormData({ email: '', password: '', role: 'user', is_active: true });
    setEditingUser(null);
  };

  const handleEdit = (user: any) => {
    setEditingUser(user);
    setFormData({
      email: user.email,
      password: '',
      role: user.role,
      is_active: user.is_active,
    });
    setShowModal(true);
  };

  const handleDelete = async (id: string) => {
    if (confirm('Are you sure you want to delete this user?')) {
      await deleteUser(id);
      fetchUsers();
    }
  };

  return (
    <div className="users-page">
      <div className="page-header">
        <h1>Users</h1>
        <button
          className="btn btn-primary"
          onClick={() => {
            resetForm();
            setShowModal(true);
          }}
        >
          <Plus size={18} />
          Add User
        </button>
      </div>

      <div className="card">
        <div className="table-container">
          <table>
            <thead>
              <tr>
                <th>Email</th>
                <th>Role</th>
                <th>Status</th>
                <th>Created</th>
                <th>Actions</th>
              </tr>
            </thead>
            <tbody>
              {users.map(user => (
                <tr key={user.id}>
                  <td>{user.email}</td>
                  <td><span className="badge badge-info">{user.role}</span></td>
                  <td>
                    <span className={`badge ${user.is_active ? 'badge-success' : 'badge-danger'}`}>
                      {user.is_active ? 'Active' : 'Inactive'}
                    </span>
                  </td>
                  <td>{new Date(user.created_at).toLocaleDateString()}</td>
                  <td className="actions">
                    <button className="btn-icon" onClick={() => handleEdit(user)}>
                      <Edit size={16} />
                    </button>
                    <button className="btn-icon" onClick={() => handleDelete(user.id)}>
                      <Trash2 size={16} />
                    </button>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>

      {showModal && (
        <div className="modal-overlay" onClick={() => setShowModal(false)}>
          <div className="modal" onClick={(e) => e.stopPropagation()}>
            <h2>{editingUser ? 'Edit User' : 'Add User'}</h2>
            <form onSubmit={handleSubmit}>
              <div className="form-group">
                <label>Email</label>
                <input
                  type="email"
                  value={formData.email}
                  onChange={(e) => setFormData({ ...formData, email: e.target.value })}
                  required
                />
              </div>

              {!editingUser && (
                <div className="form-group">
                  <label>Password</label>
                  <input
                    type="password"
                    value={formData.password}
                    onChange={(e) => setFormData({ ...formData, password: e.target.value })}
                    required
                  />
                </div>
              )}

              <div className="form-group">
                <label>Role</label>
                <select
                  value={formData.role}
                  onChange={(e) => setFormData({ ...formData, role: e.target.value })}
                >
                  <option value="user">User</option>
                  <option value="admin">Admin</option>
                </select>
              </div>

              <div className="form-group">
                <label className="checkbox-label">
                  <input
                    type="checkbox"
                    checked={formData.is_active}
                    onChange={(e) => setFormData({ ...formData, is_active: e.target.checked })}
                  />
                  Active
                </label>
              </div>

              <div className="modal-actions">
                <button type="button" className="btn btn-outline" onClick={() => setShowModal(false)}>
                  Cancel
                </button>
                <button type="submit" className="btn btn-primary">
                  {editingUser ? 'Update' : 'Create'}
                </button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  );
}
