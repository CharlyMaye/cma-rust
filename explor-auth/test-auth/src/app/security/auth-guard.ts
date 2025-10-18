import { inject } from '@angular/core';
import { CanMatchFn, Router, UrlTree } from '@angular/router';
import { Auth } from './auth';
import { take, map, Observable } from 'rxjs';

export const authGuard: CanMatchFn = (route, segments): Observable<boolean | UrlTree>  => {
  const auth = inject(Auth);
  const router = inject(Router);
  // TODO - add "loginprogress" state to avoid multiple redirects or use signals instead
  return auth.isAuthenticated$.pipe(
    take(1),
    map(isAuth => {
      console.log('Auth Guard - isAuthenticated:', isAuth);
      if (!isAuth) {
        console.log('User not authenticated, redirecting to login');
        return router.parseUrl('/login');
      }
      console.log('User authenticated, allowing access');
      return true;
    })
  );
};
