import time
from faker import Faker
from locust import HttpUser, task, between

class PostStudent(HttpUser):
    wait_time = between(1, 3)
    fake = Faker()

    @task()
    def post_student_faculty_1(self):
        for i in range(10):
            data_faculty_1 = {
                "student": self.fake.name(),
                "age": self.fake.random_int(min=18, max=25),
                "faculty": "Ingenieria",
                "discipline": self.fake.random_int(min=1, max=3)
            }

            data = data_faculty_1
            self.client.post(f"/", json=data)
            time.sleep(1)