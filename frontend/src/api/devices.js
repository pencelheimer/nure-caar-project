import client from './client'

export const getDevices    = ()              => client.get('/devices')
export const createDevice  = payload         => client.post('/devices', payload)
export const updateDevice  = (id, payload)   => client.put(`/devices/${id}`, payload)
export const deleteDevice  = id              => client.delete(`/devices/${id}`)
export const rotateKey     = id              => client.post(`/devices/${id}/rotate-key`)
export const getMeasurements = (id, params)  => client.get(`/devices/${id}/measurements`, { params })
