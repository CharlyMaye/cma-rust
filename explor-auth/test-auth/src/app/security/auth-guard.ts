import { inject } from '@angular/core';
import { CanMatchFn, Router, UrlTree } from '@angular/router';
import { Auth } from './auth';

export const authGuard: CanMatchFn = (route, segments): boolean | UrlTree => {
  const auth = inject(Auth);
  const router = inject(Router);
  
  console.log('Auth Guard - checking access for route:', route.path);
  
  if (!auth.isAuthenticated()) {
    console.log('User not authenticated, redirecting to login');
    return router.parseUrl('/login');
  }
  
  console.log('User authenticated, allowing access');
  return true;
};
