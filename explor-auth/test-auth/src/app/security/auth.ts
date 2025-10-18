import { HttpClient } from '@angular/common/http';
import { inject, Injectable, Signal, signal } from '@angular/core';
import { toSignal } from '@angular/core/rxjs-interop';
import { Router } from '@angular/router';
import { BehaviorSubject, catchError, concatMap, delay, delayWhen, map, Observable, of, switchMap, take, tap } from 'rxjs';


export abstract class Auth {
  abstract readonly isAuthenticated$: Observable<boolean>;
  abstract readonly isAuthenticated: Signal<boolean>;
  abstract login(returnUrl: string): void;
  abstract logout(): void;
  abstract checkSession(): Observable<boolean>;
}

@Injectable()
export class ConcreteAuth extends Auth {
  readonly #http = inject(HttpClient);
  readonly #router = inject(Router);
  readonly #uri = "http://localhost:8080/api/auth";

  // Options HTTP avec credentials pour les cookies
  readonly #httpOptions = {
    withCredentials: true
  };

  readonly #isAuthenticated = new BehaviorSubject<boolean>(false);
  readonly isAuthenticated$ = this.#isAuthenticated.asObservable();
  readonly isAuthenticated = toSignal(this.isAuthenticated$, {
    initialValue: this.#isAuthenticated.value
  });

  // TODO - gérer le ssr et mettre ça dans un service dédié
  private encodePassword(password: string): string {
    return btoa(password);
  }

  public login(returnUrl: string) {
    const credentials = {
      user: "test",
      password: this.encodePassword("test")
    }
    
    this.#http.post(`${this.#uri}/login`, credentials, this.#httpOptions).pipe(
      delay(100),
    ).subscribe(
      {
        next: () => {
          this.#isAuthenticated.next(true);
          this.#router.navigate([returnUrl]);
        },
        error: (err) => {
          console.error('Login failed:', err);
          this.#isAuthenticated.next(false);
        }
      });
  }

  public logout() {
    this.#isAuthenticated.next(false);
    this.#http.post(`${this.#uri}/logout`, {}, this.#httpOptions).subscribe(
      {
        next: () => {
          this.#isAuthenticated.next(false);
          this.#router.navigate(['/login']);
        },
        error: (err) => {
          console.error('Logout failed:', err);
        }
      });
  }

  // Vérifier si l'utilisateur est déjà connecté (via cookie)
  public checkSession(): Observable<boolean> {
    return this.#isAuthenticated.pipe(
      concatMap(
        (isAuthenticated) => {
          if (isAuthenticated) {
            console.log('Current isAuthenticated state:', isAuthenticated);
            return of(true);
          } else {
            console.log('Checking session with server...');
            return this.#http.get(`${this.#uri}/verify`, this.#httpOptions).pipe(
              map(() => {
                console.log('Valid session found.');
                return true;
              }),
              catchError(() => {
                console.log('No valid session found.');
                return of(false);
              })
            )
          }
        }
      ),
      tap(
        {
          next: (isAuthenticated) => {
            this.#isAuthenticated.next(isAuthenticated);
          },
          error: () => {
            this.#isAuthenticated.next(false);
          }
        },
      ),
    take(1)
    )
  }
}
