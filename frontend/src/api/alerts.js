import client from './client'

export const getAlerts = params => client.get('/alerts', { params })
