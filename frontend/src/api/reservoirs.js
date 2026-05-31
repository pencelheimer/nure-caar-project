import client from './client'

export const getReservoirs   = ()         => client.get('/reservoirs')
export const getReservoir    = id         => client.get(`/reservoirs/${id}`)
export const createReservoir = payload    => client.post('/reservoirs', payload)
export const updateReservoir = (id, payload) => client.put(`/reservoirs/${id}`, payload)
export const deleteReservoir = id         => client.delete(`/reservoirs/${id}`)

export const getRules    = reservoirId         => client.get(`/reservoirs/${reservoirId}/rules`)
export const createRule  = (reservoirId, payload) => client.post(`/reservoirs/${reservoirId}/rules`, payload)
export const updateRule  = (ruleId, payload)   => client.put(`/rules/${ruleId}`, payload)
export const deleteRule  = ruleId              => client.delete(`/rules/${ruleId}`)
