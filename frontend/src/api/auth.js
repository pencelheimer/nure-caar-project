import client from './client'

export const login = (email, password) =>
  client.post('/auth/login', { email, password })

export const register = payload =>
  client.post('/auth/register', payload)

export const me = () =>
  client.get('/auth/me')

export const updateMe = payload =>
  client.put('/auth/me', payload)

export const changePassword = payload =>
  client.post('/auth/change-password', payload)
