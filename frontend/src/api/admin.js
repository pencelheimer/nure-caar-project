import client from './client'

export const getStats = () =>
  client.get('/admin/stats')

export const getUsers = () =>
  client.get('/admin/users')

export const getUser = id =>
  client.get(`/admin/users/${id}`)

export const setUserRole = (id, role) =>
  client.put(`/admin/users/${id}/role`, null, { params: { role } })

export const banUser = (id, payload) =>
  client.post(`/admin/users/${id}/ban`, payload)

export const deleteUser = id =>
  client.delete(`/admin/users/${id}`)

export const getAuditLogs = params =>
  client.get('/admin/audit-logs', { params })
