import { Component, Input } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterOutlet, RouterLink } from '@angular/router';
import { Person } from '../person';
@Component({
  selector: 'app-person-details',
  standalone: true,
  imports: [CommonModule, RouterLink, RouterOutlet],
  templateUrl: './person-details.component.html',
  styleUrl: './person-details.component.css'
})
export class PersonDetailsComponent {
  @Input() person!: Person;
}
