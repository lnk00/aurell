export interface SessionService {
	verifyJwt: (token: string) => Promise<{ userId: string; sessionId: string }>;
}
