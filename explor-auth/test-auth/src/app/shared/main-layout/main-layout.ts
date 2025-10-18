import { ChangeDetectionStrategy, Component, inject } from '@angular/core';
import { BreakpointObserver, Breakpoints } from '@angular/cdk/layout';
import { AsyncPipe } from '@angular/common';
import { MatToolbarModule } from '@angular/material/toolbar';
import { MatButtonModule } from '@angular/material/button';
import { MatSidenavModule } from '@angular/material/sidenav';
import { MatListModule } from '@angular/material/list';
import { MatIconModule } from '@angular/material/icon';
import { Observable } from 'rxjs';
import { map, shareReplay } from 'rxjs/operators';
import { RouterOutlet } from '@angular/router';
import { Auth } from '../../security/auth';
import { Theme } from '../theme';
import { Breakpoint } from '../breakpoint';

@Component({
  selector: 'app-main-layout',
  imports: [
    RouterOutlet,
    MatToolbarModule,
    MatButtonModule,
    MatSidenavModule,
    MatListModule,
    MatIconModule,
    AsyncPipe,
  ],
  templateUrl: './main-layout.html',
  styleUrl: './main-layout.scss',
  changeDetection: ChangeDetectionStrategy.OnPush,
  host: { class: 'flex-full-size' }
})
export class MainLayout {
  private readonly _theme = inject(Theme);
  private readonly _breakpoint = inject(Breakpoint);
  private readonly _auth = inject(Auth);

  public readonly isHandset$: Observable<boolean> = this._breakpoint.isHandset$;

  public logout(): void {
    console.log('Logging out...');
    this._auth.logout();
  }

  public toggleColorScheme(): void {
    this._theme.toggleColorScheme();
  }
}
