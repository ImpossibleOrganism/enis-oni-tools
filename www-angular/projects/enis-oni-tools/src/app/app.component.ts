import {Component, OnInit} from '@angular/core';
import { RouterOutlet } from '@angular/router';
import * as oni_tools from 'oni-tools';

@Component({
  selector: 'app-root',
  imports: [RouterOutlet],
  templateUrl: './app.component.html',
  styleUrl: './app.component.sass'
})
export class AppComponent {
  title = 'enis-oni-tools';

  // async ngOnInit() {
  //  // Initialize the WASM module when the app loads
  //  await oni_tools.init();
  // }

  onButtonOne() {
    // Do a regular JS alert, to test.
    alert('Button one has been clicked!');
  }

  onButtonTwo() {
    // Call the greet function from WASM
    // alert('Button two has been clicked!');
    oni_tools.greet();
  }
}
