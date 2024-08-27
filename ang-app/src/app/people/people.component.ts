import { Component, inject } from '@angular/core';
import { CommonModule } from '@angular/common';
import { Person } from '../person';
import { RouterOutlet, RouterLink } from '@angular/router';
import { RestApiService } from '../rest-api.service';

@Component({
  selector: 'app-people',
  standalone: true,
  imports: [CommonModule, RouterLink, RouterOutlet],
  templateUrl: './people.component.html',
  styleUrl: './people.component.css'
})
export class PeopleComponent {
  filteredPeopleList: Person[] = [];

  peopleList: Person[] = [];

  restApiService: RestApiService = inject(RestApiService);


  constructor() {
    this.restApiService.getAllPeople().then((people) => {
      this.peopleList = people;
    });
  }
}
