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
  abstract checkSession(): void;
}

@Injectable()
export class ConcreteAuth extends Auth {
  readonly #http = inject(HttpClient);
  readonly #router = inject(Router);
  readonly #uri = "http://127.0.0.1:8080/api/auth";

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

  login(returnUrl: string) {
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

  logout() {
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
  checkSession() {
    this.#http.get(`${this.#uri}/verify`, this.#httpOptions).subscribe(
      {
        next: () => {
          this.#isAuthenticated.next(true);
        },
        error: () => {
          this.#isAuthenticated.next(false);
        }
      });
  }
}
