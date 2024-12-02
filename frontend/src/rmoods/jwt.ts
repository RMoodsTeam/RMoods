// Mirrors the backend version of this struct
export interface JwtClaims {
  exp: number;
  iat: number;
  userInfo: JwtUserInfo;
}

export interface JwtUserInfo {
  id: string;
}
