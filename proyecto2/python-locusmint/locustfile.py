import time
from faker import Faker
from locust import HttpUser, task, between

class PostStudent(HttpUser):
    wait_time = between(1, 3)
    fake = Faker()

    @task()
    def post_student_faculty_1(self):
        i = self.fake.random_int(min=1, max=100)
        age = self.fake.random_int(min=18, max=25)
        discipline = self.fake.random_int(min=1, max=3)
        name = self.fake.name()
        if i % 2 == 0:
            data = {
                "student": name,
                "age": age,
                "faculty": "Ingenieria",
                "discipline": discipline
            }
            self.client.post(f"/ingenieria", json=data)
        else:
            data = {
                "student": name,
                "age": age,
                "faculty": "Agronomia",
                "discipline": discipline
            }
            self.client.post(f"/agronomia", json=data)
        time.sleep(1)