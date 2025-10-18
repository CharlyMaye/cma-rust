import { HttpClient } from '@angular/common/http';
import { inject, Injectable, Signal, signal } from '@angular/core';
import { toSignal } from '@angular/core/rxjs-interop';
import { Router } from '@angular/router';
import { BehaviorSubject, delay, delayWhen, Observable } from 'rxjs';


export abstract class Auth {
  abstract readonly isAuthenticated$: Observable<boolean>;
  abstract readonly isAuthenticated: Signal<boolean>;
  abstract login(returnUrl: string): void;
  abstract logout(): void;
}

@Injectable()
export class ConcreteAuth extends Auth {
  readonly #http = inject(HttpClient);
  readonly #router = inject(Router);
  readonly #uri = "http://127.0.0.1:8080/api/auth";

  readonly #isAuthenticated = new BehaviorSubject<boolean>(false);
  readonly isAuthenticated$ = this.#isAuthenticated.asObservable();
  readonly isAuthenticated = toSignal(this.isAuthenticated$, {
    initialValue: this.#isAuthenticated.value
  });

  constructor() {
    super();
    console.log('ConcreteAuth initialized');
  }

  login(returnUrl: string) {
    this.#http.post(`${this.#uri}/login`, { /* credentials */ }).pipe(
      delay(1000),
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

  logout() {
    console.log('ConcreteAuth: User logged out');
    this.#isAuthenticated.next(false);
    this.#http.post(`${this.#uri}/logout`, {}).subscribe(
      {
        next: () => {
          this.#isAuthenticated.next(false);
          // Rediriger vers la page de login après déconnexion
          this.#router.navigate(['/login']);
        },
        error: (err) => {
          console.error('Logout failed:', err);
        }
      });
  }
}
